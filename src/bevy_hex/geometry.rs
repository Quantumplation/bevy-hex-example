use super::hex::HexCoord;

/// The ratio between a circle touching the points of a hex grid (the outer radius),
/// and a circle touching the edges of a hex grid (the inner radius).
/// Calculated as sqrt(3) / 2;
pub const HEX_INNER_RADIUS_RATIO: f32 = 0.866_025_4;

/// Generate a point located at the center of a hexagon at `c`, on a grid with hexagons of size `radius`, shifted by `offset`.
/// The parameters are used to compose larger effects like beveling
#[must_use]
pub fn center(radius: f32, c: &HexCoord, offset: &[f32; 3]) -> [f32; 3] {
    // Get floating point hex-coords
    let (qf, rf) = (c.q as f32, c.r as f32);
    // We need an outer and inner radius
    let (outer, inner) = (radius, radius * HEX_INNER_RADIUS_RATIO);

    // Start from our q coordinate,
    let start = qf;
    // Shift over by half a unit for each row
    let row_adjustment = 0.5 * rf;
    // This produces a rhombus, use integer division to cancel this out on every other row and get "roughly" a grid
    let rhombus_adjustment = -(c.r / 2) as f32;
    // Scale the whole thing up by twice the inner radius to get our x coordinate
    let x = (start + row_adjustment + rhombus_adjustment) * inner * 2.;
    // Each row moves us by 1.5 times the outer radius along the z axis
    let z = rf * outer * 1.5;

    // Return (x,0,z) shifted by the provided offset
    [x + offset[0], 0. + offset[1], z + offset[2]]
}

/// Generate a pointed located at the eastern corner of a hexagon at `c`, on a grid with hexagons of size `radius`, shifted by `offset`
#[must_use]
pub fn east_corner(radius: f32, c: &HexCoord, offset: &[f32; 3]) -> [f32; 3] {
    // Start from the center of our hexagon
    let center = center(radius, c, offset);
    // And move along the z axis for "east" by our radius
    [center[0] + 0., center[1] + 0., center[2] + radius]
}

/// Generate a pointed located at the western corner of a hexagon at `c`, on a grid with hexagons of size `radius`, shifted by `offset`
#[must_use]
pub fn west_corner(radius: f32, c: &HexCoord, offset: &[f32; 3]) -> [f32; 3] {
    // Start from the center of our hexagon
    let center = center(radius, c, offset);
    // And move along the z axis for "west" by our radius
    [center[0] + 0., center[1] + 0., center[2] - radius]
}

/// Generate a pointed located at the north-east corner of a hexagon at `c`, on a grid with hexagons of size `radius`, shifted by `offset`
#[must_use]
pub fn north_east_corner(radius: f32, c: &HexCoord, offset: &[f32; 3]) -> [f32; 3] {
    // Start from the center of our hexagon
    let center = center(radius, c, offset);
    let inner = radius * HEX_INNER_RADIUS_RATIO;
    // And move along the x axis (for "north") to be aligned with the top edge (i.e. the inner radius)
    // and along the z axis (for "east"), but not as far as the east corner
    [center[0] + inner, center[1] + 0., center[2] + 0.5 * radius]
}

/// Generate a pointed located at the north-west corner of a hexagon at `c`, on a grid with hexagons of size `radius`, shifted by `offset`
#[must_use]
pub fn north_west_corner(radius: f32, c: &HexCoord, offset: &[f32; 3]) -> [f32; 3] {
    // Start from the center of our hexagon
    let center = center(radius, c, offset);
    let inner = radius * HEX_INNER_RADIUS_RATIO;
    // And move along the x axis (for "north") to be aligned with the top edge (i.e. the inner radius)
    // and along the z axis (for "west"), but not as far as the east corner
    [center[0] + inner, center[1] + 0., center[2] - 0.5 * radius]
}

/// Generate a pointed located at the south-east corner of a hexagon at `c`, on a grid with hexagons of size `radius`, shifted by `offset`
#[must_use]
pub fn south_east_corner(radius: f32, c: &HexCoord, offset: &[f32; 3]) -> [f32; 3] {
    // Start from the center of our hexagon
    let center = center(radius, c, offset);
    let inner = radius * HEX_INNER_RADIUS_RATIO;
    // And move along the x axis (for "south") to be aligned with the top edge (i.e. the inner radius)
    // and along the z axis (for "east"), but not as far as the east corner
    [center[0] - inner, center[1] + 0., center[2] + 0.5 * radius]
}

/// Generate a pointed located at the south-west corner of a hexagon at `c`, on a grid with hexagons of size `radius`, shifted by `offset`
#[must_use]
pub fn south_west_corner(radius: f32, c: &HexCoord, offset: &[f32; 3]) -> [f32; 3] {
    // Start from the center of our hexagon
    let center = center(radius, c, offset);
    let inner = radius * HEX_INNER_RADIUS_RATIO;
    // And move along the x axis (for "south") to be aligned with the top edge (i.e. the inner radius)
    // and along the z axis (for "west"), but not as far as the east corner
    [center[0] - inner, center[1] + 0., center[2] - 0.5 * radius]
}

/// Fill `pts` with the points around the edge of a flat hexagon of a specific radius at a specific coordinate
pub fn flat_hexagon_ring(pts: &mut Vec<[f32; 3]>, radius: f32, c: &HexCoord, offset: &[f32; 3]) {
    pts.extend(
        [
            east_corner(radius, c, offset), // Each of the corners, counter-clockwise from the east corner
            north_east_corner(radius, c, offset), // ...
            north_west_corner(radius, c, offset), // ...
            west_corner(radius, c, offset), // ...
            south_west_corner(radius, c, offset), // ...
            south_east_corner(radius, c, offset), // ...
            east_corner(radius, c, offset), // We include the east corner an extra time,
                                            // so we don't have to mess around with modulus
        ]
        .iter(),
    );
}

/// Fill `pts` with the points of a flat hexagon of a specific radius at a specific coordinate
pub fn flat_hexagon_points(pts: &mut Vec<[f32; 3]>, radius: f32, c: &HexCoord) {
    // We'll create 6 triangles, all sharing a center point
    pts.push(center(radius, c, &[0., 0., 0.]));
    flat_hexagon_ring(pts, radius, c, &[0., 0., 0.]);
}

/// Fill `normals` with the normals for a flat hexagon
pub fn flat_hexagon_normals(normals: &mut Vec<[f32; 3]>) {
    // Each of the 8 points (center + corners + repeat) just points up
    for _ in 0..8 {
        normals.push([0., 1., 0.]);
    }
}

/// Fill `idx` with the indices to create a hexagon when interpreted as a triangle list
pub fn flat_hexagon_indices(idx: &mut Vec<u32>) {
    // Each of the six faces
    for i in 0..=6 {
        //           first-time     second-time
        idx.push(0); // Center
        idx.push(i + 1); // Point       East           North-east
        idx.push(i + 2); // Next point  North-east     North-west
    }
}

/// Fill `points` with the points for a beveled `radius` hexagon, beveled by `factor`, at point `c`
pub fn bevel_hexagon_points(points: &mut Vec<[f32; 3]>, radius: f32, factor: f32, c: &HexCoord) {
    let inner_radius = radius * factor;
    // Populate the points for the top face, as a slightly scaled hexagon
    flat_hexagon_points(points, inner_radius, c);

    // We want to insert a full sized hexagon slightly below the face,
    // offset by the same distance we scaled in, so the slopes are 45 degrees
    let offset = [0., inner_radius - radius, 0.];

    // Add small slopes
    flat_hexagon_ring(points, radius, c, &offset);

    // Now, add points much lower, so we can create skirts so if hexagons are offset we don't see gaps
    let offset = [0., -10., 0.];
    // Add skirts
    flat_hexagon_ring(points, radius, c, &offset);
}

/// Fill `normals` with the normals for the a beveled hexagon
pub fn bevel_hexagon_normals(normals: &mut Vec<[f32; 3]>) {
    // Fill in the normals for the flat top
    flat_hexagon_normals(normals);
    // Fake a coordinate, since we don't need it for normals
    let c = &HexCoord::new(0, 0);
    // If we create a tiny hexagon, and lift those points up, the resulting vectors will be normals orthogonal to our 45 degree slopes
    let offset = [0., 0.707, 0.];
    flat_hexagon_ring(normals, 0.707, c, &offset);
    // Similarly, if we do a 1-radius hexagon, this will give us points pointing outward for our skirts
    let offset = [0., 0., 0.];
    flat_hexagon_ring(normals, 1., c, &offset);
}

/// Fill `idx` with indices to draw a quad using the 4 provided corners
pub fn quad_indices(
    idx: &mut Vec<u32>,
    top_left: u32,
    top_right: u32,
    bottom_left: u32,
    bottom_right: u32,
) {
    // First triangle
    idx.extend([top_left, bottom_left, bottom_right].iter());
    // Second triangle
    idx.extend([top_left, bottom_right, top_right].iter());
}

/// Fill `idx` with indices to construct a beveled hexagon
pub fn bevel_hexagon_indices(idx: &mut Vec<u32>) {
    // First, fill indices with the flat top hexagon
    flat_hexagon_indices(idx);

    // Add slopes
    for i in 0..=6 {
        // Insert a quad, using the inner beveled hex, and the outer sloped hex
        quad_indices(idx, i + 1, i + 2, i + 8, i + 9);
    }
    // Add a skirt
    for i in 0..=6 {
        // Insert a quad using the outer sloped hex and the bottom base hex
        quad_indices(idx, i + 8, i + 9, i + 15, i + 16);
    }
}
