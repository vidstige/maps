use crate::grid::Grid;

pub trait Field {
    fn value(&self, x: f64, y: f64) -> f64;
}

pub type Contour = Vec<[f64; 2]>;

const POINTS: [[f64; 2]; 4] = [
    [0.0, 0.0],
    [1.0, 0.0],
    [0.0, 1.0],
    [1.0, 1.0],
];

type EdgeIndices = (usize, usize);
const LOOKUP: [[Option<(EdgeIndices, EdgeIndices)>; 2]; 16] = [
    [None, None],
    [Some(((0, 2), (0, 1))), None],
    [Some(((0, 1), (1, 3))), None],
    [Some(((0, 2), (1, 3))), None],
    [Some(((0, 2), (2, 3))), None],
    [Some(((0, 1), (2, 3))), None],
    [Some(((0, 2), (2, 3))), Some(((0, 1), (1, 3)))],
    [Some(((2, 3), (1, 3))), None],
    [Some(((1, 3), (2, 3))), None],
    [Some(((0, 2), (0, 1))), Some(((2, 3), (1, 3)))],
    [Some(((2, 3), (0, 1))), None],
    [Some(((2, 3), (0, 2))), None],
    [Some(((1, 3), (0, 2))), None],
    [Some(((1, 3), (0, 1))), None],
    [Some(((0, 1), (0, 2))), None],
    [None, None],
];

/*
def interpolate_edge(points: np.ndarray, values: np.ndarray, edge: Edge, level: float) -> np.ndarray:
    i0, i1 = edge
    v0, v1 = values[i0], values[i1]
    # make sure v0 is smaller that v1
    if v0 > v1:
        i0, i1 = i1, i0
        v0, v1 = v1, v0

    # compute parameter
    t = (level - v0) / (v1 - v0)
    # lerp
    return (1 - t) * points[i0] + t * points[i1]


def marching_squares(sdf: np.ndarray, level: float = .0) -> Tuple[np.ndarray, np.ndarray]:
    h, w = sdf.shape
    vertices = []
    lines = []
    for p, block in iter_blocks(sdf, (2, 2)):
        values = block.ravel()
        index = sum(int(bit) * 1 << i for i, bit in enumerate(values < level))
        for e0, e1 in LOOKUP[index]:
            a, b = interpolate_edge(POINTS, values, e0, level), interpolate_edge(POINTS, values, e1, level)
            lines.append((len(vertices), len(vertices) + 1))
            vertices.extend([p + a, p + b])

    return np.vstack(vertices), lines
 */
fn interpolate_edge(points: &[[f64; 2]], values: &[f64; 4], (i0, i1): EdgeIndices, level: f64) -> [f64; 2] {
    // make sure v0 is smaller that v1
    let (v0, v1) = if values[i0] > values[i1] {
        (values[i0], values[i1])
    } else {
        (values[i1], values[i0])
    };
    // compute parameter
    let t = (level - v0) / (v1 - v0);
    // lerp x & y
    [
        (1.0 - t) * points[i0][0] + t * points[i1][0],
        (1.0 - t) * points[i0][1] + t * points[i1][1],
    ]
    
}

pub fn find_contours(grid: &Grid<f64>, level: f64) -> Vec<Contour> {
    let mut vertices = Vec::new();
    let mut lines = Vec::new();
    for y in 0..grid.height() - 1 {
        for x in 0..grid.width() - 1 {
            // find grid values at the corner of the square
            let values = [
                grid[(x, y)],
                grid[(x + 1, y)],
                grid[(x, y + 1)],
                grid[(x + 1, y + 1)]
            ];
            
            // find table index
            let table_index: usize = values
                .iter()
                .enumerate()
                .map(|(i, v)| (1 << i) * ((*v < level) as usize)).sum();
            // loop over items that are not None
            for (e0, e1) in LOOKUP[table_index].iter().flatten() {
                let a = interpolate_edge(&POINTS, &values, *e0, level);
                let b = interpolate_edge(&POINTS, &values, *e1, level);
                lines.push((vertices.len(), vertices.len() + 1));
                vertices.push([x as f64 + a[0], y as f64 + a[1]]);
                vertices.push([x as f64 + b[0], y as f64 + b[1]]);
            }
        }
    }
    // just dump all line segments
    lines.iter().map(|(i0, i1)| vec![vertices[*i0], vertices[*i1]]).collect()
}
