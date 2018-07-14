use dimensions::Pixels;
use geometry::{
    Mesh,
    QuadMeshBuilder
};
use glm;
use image::Png;
use std::{
    collections::BTreeSet,
    self,
};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Coord<T> {
    row: Pixels,
    col: Pixels,
    _phantom: std::marker::PhantomData<T>,
}

impl <T> Coord<T> {
    pub fn new(row: Pixels, col: Pixels) -> Coord<T> {
        Coord {
            row,
            col,
            _phantom: std::marker::PhantomData
        }
    }

    pub fn offset(&self, dr: i64, dc: i64) -> Coord<T> {
        Coord {
            row: (self.row as i64 + dr) as Pixels,
            col: (self.col as i64 + dc) as Pixels,
            _phantom: std::marker::PhantomData,
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Image;
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Edge;
type ImageCoord = Coord<Image>;
type EdgeCoord = Coord<Edge>;

#[derive(Copy, Clone)]
struct WallInfo {
    right: bool,
    bottom: bool,
}

impl WallInfo {
    pub fn new() -> WallInfo {
        WallInfo {
            right: false,
            bottom: false,
        }
    }
}

fn is_empty_pixel(png: &Png, coord: ImageCoord) -> bool {
    png.img[coord.row][coord.col].a < 0.00001
}

fn is_border_cell(png: &Png, coord: &ImageCoord) -> bool {
    if is_empty_pixel(png, *coord) {
        return false;
    }
    for dr in -1..1 + 1 {
        for dc in -1..1 + 1 {
            if !(dr == 0 && dc == 0) {
                if is_empty_pixel(png, coord.offset(dr, dc)) {
                    return true;
                }
            }
        }
    }
    false
}

fn border_cells(png: &Png) -> Vec<ImageCoord> {
    let (width, height) = png.size();
    let mut border_cells = Vec::with_capacity(width);
    // Exclude the image edges
    for row in 1..height-1 {
        for col in 1..width-1 {
            let i_coord = ImageCoord::new(row, col);
            if is_border_cell(png, &i_coord) {
                border_cells.push(i_coord);
            }
        }
    }
    border_cells
}

fn set_walls_horizontal(png: &Png, focus: ImageCoord, focus_row: Pixels, walls: &mut Vec<Vec<WallInfo>>) {
    let (width, _) = png.size();
    for col in focus.col..width - 1 {
        if is_empty_pixel(png, ImageCoord::new(focus.row, col)) {
            break;
        }
        walls[focus_row][col].bottom = true;
    }
    for col in (1..focus.col).rev() {
        if is_empty_pixel(png, ImageCoord::new(focus.row, col)) {
            break;
        }
        walls[focus_row][col].bottom = true;
    }
}

fn set_walls_vertical(png: &Png, focus: ImageCoord, focus_col: Pixels, walls: &mut Vec<Vec<WallInfo>>) {
    let (_, height) = png.size();
    for row in focus.row..height - 1 {
        if is_empty_pixel(png, ImageCoord::new(row, focus.col)) {
            break;
        }
        walls[row][focus_col].right = true;
    }
    for row in (1..focus.row).rev() {
        if is_empty_pixel(png, ImageCoord::new(row, focus.col)) {
            break;
        }
        walls[row][focus_col].right = true;
    }
}

fn cell_walls(png: &Png, border_cells: Vec<ImageCoord>) -> Vec<Vec<WallInfo>> {
    let (width, height) = png.size();
    let mut walls = Vec::with_capacity(height);
    let default_wall_info = WallInfo::new();
    let row = vec![default_wall_info.clone(); width];
    for _ in 0..height {
        walls.push(row.clone());
    }
    for focus in border_cells.into_iter() {
        if is_empty_pixel(png, focus.offset(-1, 0)) {
            set_walls_horizontal(png, focus, focus.row - 1, &mut walls);
        }
        if is_empty_pixel(png, focus.offset(1, 0)) {
            set_walls_horizontal(png, focus, focus.row, &mut walls);
        }
        if is_empty_pixel(png, focus.offset(0, -1)) {
            set_walls_vertical(png, focus, focus.col - 1, &mut walls);
        }
        if is_empty_pixel(png, focus.offset(0, 1)) {
            set_walls_vertical(png, focus, focus.col, &mut walls);
        }
    }
    walls
}

fn edge_points(walls: Vec<Vec<WallInfo>>) -> BTreeSet<EdgeCoord> {
    let mut edges = BTreeSet::new();
    let (width, height) = (walls[0].len(), walls.len());
    for row in 1..height - 1 {
        for col in 1..width - 1 {
            if walls[row - 1][col].bottom && walls[row][col - 1].right {
                edges.insert(EdgeCoord::new(row - 1, col - 1));
            }
            if walls[row - 1][col].bottom && walls[row][col].right {
                edges.insert(EdgeCoord::new(row - 1, col));
            }
            if walls[row][col].bottom && walls[row][col - 1].right {
                edges.insert(EdgeCoord::new(row, col - 1));
            }
            if walls[row][col].bottom && walls[row][col].right {
                edges.insert(EdgeCoord::new(row, col));
            }
        }
    }
    edges
}

fn is_blank(png: &Png, coord: EdgeCoord) -> bool {
    is_empty_pixel(png, ImageCoord::new(coord.row, coord.col))
}

fn try_find_bottom(png: &Png, features: &BTreeSet<EdgeCoord>, edge: EdgeCoord) -> Option<EdgeCoord> {
    let (_, height) = png.size();
    for row in edge.row + 1..height - 1 {
        let e = EdgeCoord::new(row, edge.col);
        if is_blank(png, e) && is_blank(png, e.offset(0, 1)) {
            break;
        } else if features.contains(&e) {
            return Some(e);
        }
    }
    None
}

fn try_find_right(png: &Png, features: &BTreeSet<EdgeCoord>, edge: EdgeCoord) -> Option<EdgeCoord> {
    let (width, _) = png.size();
    for col in edge.col + 1..width - 1 {
        let e = EdgeCoord::new(edge.row, col);
        if is_blank(png, e) && is_blank(png, e.offset(1, 0)) {
            break;
        } else if features.contains(&e) {
            return Some(e);
        }
    }
    None
}

fn basic_cartesian(png: &Png, z_value: f32, edge: EdgeCoord) -> glm::Vec3 {
    let mut offset: (i64, i64) = (0, 0);
    if !is_blank(png, edge) {
        offset.0 -= 1;
        offset.1 -= 1;
    }
    if !is_blank(png, edge.offset(1,0)) {
        offset.0 += 1;
        offset.1 -= 1;
    }
    if !is_blank(png, edge.offset(0,1)) {
        offset.0 -= 1;
        offset.1 += 1;
    }
    if !is_blank(png, edge.offset(1,1)) {
        offset.0 += 1;
        offset.1 += 1;
    }
    let dr = offset.0 as f32 * 0.001;
    let dc = offset.1 as f32 * 0.001;
    let (_, height) = png.size();
    glm::vec3((edge.col + 1) as f32 + dc,
              (height - edge.row - 1) as f32 - dr,
              z_value)
}

fn finalize(png: Png, features: BTreeSet<EdgeCoord>) -> Mesh {
    let mut builder = QuadMeshBuilder::new();
    for feature in features.iter() {
        let bottom = try_find_bottom(&png, &features, *feature);
        let right = try_find_right(&png, &features, *feature);
        let blank_tr = is_blank(&png, feature.offset(0, 1));
        let blank_br = is_blank(&png, feature.offset(1, 1));
        let blank_bl = is_blank(&png, feature.offset(1, 0));

        if let (Some(ref bottom), Some(ref right)) = (bottom, right) {
            if !blank_br {
                let bottom_right = EdgeCoord::new(bottom.row, right.col);
                let tl = basic_cartesian(&png, 0.0, *feature);
                let tr = basic_cartesian(&png, 0.0, *right);
                let bl = basic_cartesian(&png, 0.0,*bottom);
                let br = basic_cartesian(&png, 0.0, bottom_right);
                builder.add_face(tl, tr, bl, br);
            }
        }
        if let Some(ref bottom) = bottom {
            if blank_bl || blank_br {
                let t = basic_cartesian(&png, 0.0, *feature);
                let b = basic_cartesian(&png, 0.0, *bottom);
                let tz = basic_cartesian(&png, -1.0,*feature);
                let bz = basic_cartesian(&png, -1.0, *bottom);
                if blank_bl {
                    builder.add_face(bz, tz, b, t);
                } else {
                    builder.add_face(tz, bz, t, b);
                }
            }
        }
        if let Some(ref right) = right {
            if blank_tr || blank_br {
                let l = basic_cartesian(&png, 0.0, *feature);
                let r = basic_cartesian(&png, 0.0, *right);
                let lz = basic_cartesian(&png, -1.0,*feature);
                let rz = basic_cartesian(&png, -1.0, *right);
                if blank_tr {
                    builder.add_face(lz, rz, l, r);
                } else {
                    builder.add_face(rz, lz, r, l);
                }
            }
        }
    }
    builder.build()
}

pub fn from_image(png: Png) -> Mesh {
    let border_cells = border_cells(&png);
    let walls = cell_walls(&png, border_cells);
    let features = edge_points(walls);
    finalize(png, features)
}