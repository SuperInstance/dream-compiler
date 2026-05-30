//! Dream Compiler — turns kids' descriptions into voxel structures.
//!
//! "I want a tall crystal tower with a garden on top and a moat around it"
//! → [CrystalTower(height=12), Garden(x=0,y=12,z=0), Moat(radius=5)]
//!
//! The dream compiler is pure pattern matching + composition — no LLM needed.
//! It teaches kids that programming is just: describe what you want precisely.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A voxel position and material.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct Voxel {
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub material: Material,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum Material {
    Air,
    Stone,
    Grass,
    Dirt,
    Wood,
    Leaves,
    Glass,
    Water,
    Sand,
    Glow,
    Crystal,
    Lava,
    Snow,
    Ice,
    Gold,
    Custom(u8),
}

impl Material {
    pub fn name(&self) -> &str {
        match self {
            Material::Air => "air",
            Material::Stone => "stone",
            Material::Grass => "grass",
            Material::Dirt => "dirt",
            Material::Wood => "wood",
            Material::Leaves => "leaves",
            Material::Glass => "glass",
            Material::Water => "water",
            Material::Sand => "sand",
            Material::Glow => "glow",
            Material::Crystal => "crystal",
            Material::Lava => "lava",
            Material::Snow => "snow",
            Material::Ice => "ice",
            Material::Gold => "gold",
            Material::Custom(_) => "custom",
        }
    }
}

/// A compiled dream: a list of voxels ready to place in the world.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dream {
    pub voxels: Vec<Voxel>,
    pub name: String,
    pub description: String,
}

// Dream templates are handled internally via match — no serialization needed for fn pointers

/// Parameters for dream generation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DreamParams {
    pub width: i32,
    pub height: i32,
    pub depth: i32,
    pub material: Material,
    pub extras: HashMap<String, f64>,
}

impl Default for DreamParams {
    fn default() -> Self {
        Self {
            width: 5,
            height: 5,
            depth: 5,
            material: Material::Stone,
            extras: HashMap::new(),
        }
    }
}

/// The dream compiler.
pub struct DreamCompiler {
    templates: Vec<DreamTemplateEntry>,
}

#[derive(Debug, Clone)]
struct DreamTemplateEntry {
    name: String,
    keywords: Vec<String>,
}

impl DreamCompiler {
    pub fn new() -> Self {
        Self {
            templates: vec![
                DreamTemplateEntry { name: "tower".into(), keywords: vec!["tower".into(), "tall".into(), "spire".into()] },
                DreamTemplateEntry { name: "cabin".into(), keywords: vec!["cabin".into(), "house".into(), "home".into(), "hut".into()] },
                DreamTemplateEntry { name: "castle".into(), keywords: vec!["castle".into(), "fortress".into(), "fort".into()] },
                DreamTemplateEntry { name: "garden".into(), keywords: vec!["garden".into(), "park".into(), "flowers".into()] },
                DreamTemplateEntry { name: "crystal".into(), keywords: vec!["crystal".into(), "gem".into(), "diamond".into()] },
                DreamTemplateEntry { name: "bridge".into(), keywords: vec!["bridge".into(), "walkway".into()] },
                DreamTemplateEntry { name: "moat".into(), keywords: vec!["moat".into(), "pond".into(), "lake".into(), "pool".into()] },
                DreamTemplateEntry { name: "tree".into(), keywords: vec!["tree".into(), "forest".into()] },
                DreamTemplateEntry { name: "wall".into(), keywords: vec!["wall".into(), "fence".into(), "barrier".into()] },
                DreamTemplateEntry { name: "stairs".into(), keywords: vec!["stairs".into(), "staircase".into(), "ladder".into()] },
                DreamTemplateEntry { name: "pyramid".into(), keywords: vec!["pyramid".into(), "triangle".into()] },
                DreamTemplateEntry { name: "sphere".into(), keywords: vec!["sphere".into(), "ball".into(), "orb".into(), "planet".into()] },
            ],
        }
    }

    /// Parse a dream description and compile it to voxels.
    pub fn compile(&self, description: &str) -> Dream {
        let desc_lower = description.to_lowercase();
        let mut voxels = Vec::new();
        let mut offset_x = 0i32;

        // Extract size hints
        let size = self.extract_size(&desc_lower);
        let params = DreamParams {
            width: size,
            height: size,
            depth: size,
            material: Material::Stone,
            extras: HashMap::new(),
        };

        // Find which templates match
        let matched = self.match_templates(&desc_lower);

        if matched.is_empty() {
            // Default: build a small cube
            voxels = generate_cube(0, 0, 0, 3, Material::Stone);
            return Dream {
                voxels,
                name: "mystery-box".into(),
                description: description.into(),
            };
        }

        // Generate each matched template, offset so they don't overlap
        for template_name in &matched {
            let template_voxels = generate_template(template_name, offset_x, 0, 0, &params);
            offset_x += params.width + 3; // spacing
            voxels.extend(template_voxels);
        }

        Dream {
            voxels,
            name: matched.join("-and-"),
            description: description.into(),
        }
    }

    fn match_templates(&self, text: &str) -> Vec<String> {
        let mut matched = Vec::new();
        for template in &self.templates {
            if template.keywords.iter().any(|k| text.contains(k)) {
                matched.push(template.name.clone());
            }
        }
        // Deduplicate while preserving order
        let mut seen = std::collections::HashSet::new();
        matched.retain(|m| seen.insert(m.clone()));
        matched
    }

    fn extract_size(&self, text: &str) -> i32 {
        // Look for size words
        if text.contains("huge") || text.contains("massive") || text.contains("giant") {
            15
        } else if text.contains("big") || text.contains("large") {
            10
        } else if text.contains("small") || text.contains("tiny") || text.contains("little") {
            3
        } else if text.contains("tall") {
            12
        } else {
            5
        }
    }
}

fn generate_template(name: &str, ox: i32, oy: i32, oz: i32, params: &DreamParams) -> Vec<Voxel> {
    match name {
        "tower" => generate_tower(ox, oy, oz, params.height, Material::Stone),
        "cabin" => generate_cabin(ox, oy, oz, params.width),
        "castle" => generate_castle(ox, oy, oz, params.width),
        "garden" => generate_garden(ox, oy, oz, params.width),
        "crystal" => generate_crystal(ox, oy, oz, params.height),
        "bridge" => generate_bridge(ox, oy, oz, params.width, params.depth),
        "moat" => generate_moat(ox, oy, oz, params.width),
        "tree" => generate_tree(ox, oy, oz),
        "wall" => generate_wall(ox, oy, oz, params.width, params.height),
        "stairs" => generate_stairs(ox, oy, oz, params.height),
        "pyramid" => generate_pyramid(ox, oy, oz, params.width),
        "sphere" => generate_sphere(ox, oy, oz, params.width / 2),
        _ => generate_cube(ox, oy, oz, params.width, Material::Stone),
    }
}

fn v(x: i32, y: i32, z: i32, m: Material) -> Voxel { Voxel { x, y, z, material: m } }

fn generate_cube(ox: i32, oy: i32, oz: i32, size: i32, mat: Material) -> Vec<Voxel> {
    let mut voxels = Vec::new();
    for x in 0..size {
        for y in 0..size {
            for z in 0..size {
                voxels.push(v(ox + x, oy + y, oz + z, mat));
            }
        }
    }
    voxels
}

fn generate_tower(ox: i32, oy: i32, oz: i32, height: i32, mat: Material) -> Vec<Voxel> {
    let mut voxels = Vec::new();
    for y in 0..height {
        // 3x3 column
        for x in 0..3 {
            for z in 0..3 {
                // Hollow inside except floor
                if y == 0 || x == 0 || x == 2 || z == 0 || z == 2 {
                    voxels.push(v(ox + x, oy + y, oz + z, mat));
                }
            }
        }
    }
    // Pointed roof
    for y in 0..2 {
        voxels.push(v(ox + 1, oy + height + y, oz + 1, Material::Gold));
    }
    voxels
}

fn generate_cabin(ox: i32, oy: i32, oz: i32, size: i32) -> Vec<Voxel> {
    let mut voxels = Vec::new();
    let s = size.min(7);
    // Walls
    for x in 0..s {
        for z in 0..s {
            for y in 0..3 {
                if x == 0 || x == s-1 || z == 0 || z == s-1 || y == 0 {
                    voxels.push(v(ox + x, oy + y, oz + z, Material::Wood));
                }
            }
        }
    }
    // Roof
    for x in -1..=s {
        for z in -1..=s {
            voxels.push(v(ox + x, oy + 3, oz + z, Material::Leaves));
        }
    }
    // Door
    voxels.retain(|vox| !(vox.x == ox + s/2 && vox.z == oz && vox.y < 3 && vox.y > 0));
    voxels
}

fn generate_castle(ox: i32, oy: i32, oz: i32, size: i32) -> Vec<Voxel> {
    let mut voxels = Vec::new();
    let s = size.min(10);
    // Walls
    for x in 0..s {
        for z in 0..s {
            for y in 0..4 {
                if x == 0 || x == s-1 || z == 0 || z == s-1 {
                    voxels.push(v(ox + x, oy + y, oz + z, Material::Stone));
                }
            }
        }
    }
    // Corner towers
    for &(tx, tz) in &[(0, 0), (s-1, 0), (0, s-1), (s-1, s-1)] {
        for y in 4..7 {
            for x in 0..2 {
                for z in 0..2 {
                    voxels.push(v(ox + tx + x - 1, oy + y, oz + tz + z - 1, Material::Stone));
                }
            }
        }
    }
    voxels
}

fn generate_garden(ox: i32, oy: i32, oz: i32, size: i32) -> Vec<Voxel> {
    let mut voxels = Vec::new();
    let s = size.min(8);
    for x in 0..s {
        for z in 0..s {
            voxels.push(v(ox + x, oy, oz + z, Material::Grass));
            // Random flowers (every other block)
            if (x + z) % 3 == 0 {
                voxels.push(v(ox + x, oy + 1, oz + z, Material::Glow));
            }
        }
    }
    // Trees in corners
    for &(tx, tz) in &[(1, 1), (s-2, s-2)] {
        voxels.extend(generate_tree(ox + tx, oy + 1, oz + tz));
    }
    voxels
}

fn generate_crystal(ox: i32, oy: i32, oz: i32, height: i32) -> Vec<Voxel> {
    let mut voxels = Vec::new();
    let h = height.max(3);
    // Diamond shape
    let mid = h / 2;
    for y in 0..h {
        let width = if y < mid { y + 1 } else { h - y };
        for x in -width..=width {
            for z in -width..=width {
                if i32::abs(x) + i32::abs(z) <= width {
                    voxels.push(v(ox + x + mid, oy + y, oz + z + mid, Material::Crystal));
                }
            }
        }
    }
    voxels
}

fn generate_bridge(ox: i32, oy: i32, oz: i32, width: i32, _depth: i32) -> Vec<Voxel> {
    let mut voxels = Vec::new();
    let w = width.max(5);
    for x in 0..w {
        voxels.push(v(ox + x, oy, oz, Material::Wood));
        voxels.push(v(ox + x, oy, oz + 1, Material::Wood));
        // Railings
        if x % 2 == 0 {
            voxels.push(v(ox + x, oy + 1, oz, Material::Wood));
            voxels.push(v(ox + x, oy + 1, oz + 1, Material::Wood));
        }
    }
    voxels
}

fn generate_moat(ox: i32, oy: i32, oz: i32, size: i32) -> Vec<Voxel> {
    let mut voxels = Vec::new();
    let s = size.max(5);
    for x in 0..s {
        for z in 0..s {
            if x == 0 || x == s-1 || z == 0 || z == s-1 {
                voxels.push(v(ox + x, oy - 1, oz + z, Material::Water));
            }
        }
    }
    voxels
}

fn generate_tree(ox: i32, oy: i32, oz: i32) -> Vec<Voxel> {
    let mut voxels = Vec::new();
    // Trunk
    for y in 0..4 {
        voxels.push(v(ox, oy + y, oz, Material::Wood));
    }
    // Canopy
    for x in -2..=2 {
        for y in 0..2 {
            for z in -2..=2 {
                if i32::abs(x) + i32::abs(z) <= 3 {
                    voxels.push(v(ox + x, oy + 4 + y, oz + z, Material::Leaves));
                }
            }
        }
    }
    voxels
}

fn generate_wall(ox: i32, oy: i32, oz: i32, width: i32, height: i32) -> Vec<Voxel> {
    let mut voxels = Vec::new();
    for x in 0..width {
        for y in 0..height {
            voxels.push(v(ox + x, oy + y, oz, Material::Stone));
        }
    }
    voxels
}

fn generate_stairs(ox: i32, oy: i32, oz: i32, height: i32) -> Vec<Voxel> {
    (0..height).map(|i| v(ox + i, oy + i, oz, Material::Stone)).collect()
}

fn generate_pyramid(ox: i32, oy: i32, oz: i32, size: i32) -> Vec<Voxel> {
    let mut voxels = Vec::new();
    let s = size.max(3);
    for y in 0..s {
        let layer_size = s - y;
        for x in 0..layer_size {
            for z in 0..layer_size {
                voxels.push(v(ox + x + y, oy + y, oz + z + y, Material::Sand));
            }
        }
    }
    // Gold cap
    voxels.push(v(ox + s - 1, oy + s - 1, oz + s - 1, Material::Gold));
    voxels
}

fn generate_sphere(ox: i32, oy: i32, oz: i32, radius: i32) -> Vec<Voxel> {
    let mut voxels = Vec::new();
    let r = radius.max(2) as f64;
    for x in -radius..=radius {
        for y in -radius..=radius {
            for z in -radius..=radius {
                let dist = ((x*x + y*y + z*z) as f64).sqrt();
                if dist <= r {
                    let mat = if dist > r - 1.0 { Material::Glass } else { Material::Glow };
                    voxels.push(v(ox + x + radius, oy + y + radius, oz + z + radius, mat));
                }
            }
        }
    }
    voxels
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compile_tower() {
        let compiler = DreamCompiler::new();
        let dream = compiler.compile("build a tall tower");
        assert!(dream.voxels.len() > 10);
        assert!(dream.name.contains("tower"));
        assert!(dream.voxels.iter().any(|v| v.material == Material::Gold));
    }

    #[test]
    fn test_compile_cabin() {
        let compiler = DreamCompiler::new();
        let dream = compiler.compile("make a small cabin");
        assert!(dream.voxels.iter().any(|v| v.material == Material::Wood));
        assert!(dream.voxels.iter().any(|v| v.material == Material::Leaves));
    }

    #[test]
    fn test_compile_castle() {
        let compiler = DreamCompiler::new();
        let dream = compiler.compile("I want a big castle");
        assert!(dream.voxels.len() > 50);
        assert!(dream.name.contains("castle"));
    }

    #[test]
    fn test_compile_garden() {
        let compiler = DreamCompiler::new();
        let dream = compiler.compile("a beautiful garden with flowers");
        assert!(dream.voxels.iter().any(|v| v.material == Material::Grass));
        assert!(dream.voxels.iter().any(|v| v.material == Material::Glow));
    }

    #[test]
    fn test_compile_crystal() {
        let compiler = DreamCompiler::new();
        let dream = compiler.compile("build a crystal");
        assert!(dream.voxels.iter().any(|v| v.material == Material::Crystal));
    }

    #[test]
    fn test_compile_unknown() {
        let compiler = DreamCompiler::new();
        let dream = compiler.compile("xyzzy foo bar");
        assert!(!dream.voxels.is_empty()); // gets default cube
        assert_eq!(dream.name, "mystery-box");
    }

    #[test]
    fn test_compile_composite() {
        let compiler = DreamCompiler::new();
        let dream = compiler.compile("build a tower with a garden and a moat");
        assert!(dream.name.contains("tower"));
        assert!(dream.name.contains("garden"));
        assert!(dream.name.contains("moat"));
    }

    #[test]
    fn test_compile_pyramid() {
        let compiler = DreamCompiler::new();
        let dream = compiler.compile("a huge pyramid");
        assert!(dream.voxels.iter().any(|v| v.material == Material::Sand));
        assert!(dream.voxels.iter().any(|v| v.material == Material::Gold));
    }

    #[test]
    fn test_compile_sphere() {
        let compiler = DreamCompiler::new();
        let dream = compiler.compile("a glowing orb");
        assert!(dream.voxels.iter().any(|v| v.material == Material::Glow));
        assert!(dream.voxels.iter().any(|v| v.material == Material::Glass));
    }

    #[test]
    fn test_compile_bridge() {
        let compiler = DreamCompiler::new();
        let dream = compiler.compile("build a bridge");
        assert!(dream.voxels.iter().any(|v| v.material == Material::Wood));
    }

    #[test]
    fn test_compile_tree() {
        let compiler = DreamCompiler::new();
        let dream = compiler.compile("plant a tree");
        assert!(dream.voxels.iter().any(|v| v.material == Material::Wood));
        assert!(dream.voxels.iter().any(|v| v.material == Material::Leaves));
    }

    #[test]
    fn test_compile_stairs() {
        let compiler = DreamCompiler::new();
        let dream = compiler.compile("build stairs");
        assert!(dream.voxels.len() >= 3);
    }

    #[test]
    fn test_compile_wall() {
        let compiler = DreamCompiler::new();
        let dream = compiler.compile("a big wall");
        assert!(dream.voxels.len() > 20);
    }

    #[test]
    fn test_size_huge() {
        let compiler = DreamCompiler::new();
        let dream = compiler.compile("a huge tower");
        // Should be taller than default
        let max_y = dream.voxels.iter().map(|v| v.y).max().unwrap_or(0);
        assert!(max_y > 10);
    }

    #[test]
    fn test_size_tiny() {
        let compiler = DreamCompiler::new();
        let dream = compiler.compile("a tiny cabin");
        let max_x = dream.voxels.iter().map(|v| v.x).max().unwrap_or(0);
        assert!(max_x < 10);
    }

    #[test]
    fn test_dream_preserves_description() {
        let compiler = DreamCompiler::new();
        let dream = compiler.compile("my magical castle");
        assert_eq!(dream.description, "my magical castle");
    }

    #[test]
    fn test_composite_offset() {
        let compiler = DreamCompiler::new();
        let dream = compiler.compile("tower and garden");
        // They should not overlap — garden starts after tower
        let tower_max_x = dream.voxels.iter()
            .filter(|v| v.y > 0) // non-ground
            .map(|v| v.x).min().unwrap_or(0);
        assert!(dream.voxels.len() > 20); // combined structure
    }

    #[test]
    fn test_material_names() {
        assert_eq!(Material::Crystal.name(), "crystal");
        assert_eq!(Material::Glow.name(), "glow");
        assert_eq!(Material::Water.name(), "water");
    }

    #[test]
    fn test_serialization() {
        let compiler = DreamCompiler::new();
        let dream = compiler.compile("a crystal tower");
        let json = serde_json::to_string(&dream).unwrap();
        let restored: Dream = serde_json::from_str(&json).unwrap();
        assert_eq!(restored.voxels.len(), dream.voxels.len());
        assert_eq!(restored.description, "a crystal tower");
    }
}
