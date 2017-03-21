use std::io::{self, Write};
use Smd;

pub trait SmdExportExt {
    fn export<W: Write>(&self, target: W) -> Result<(), io::Error>;
}

impl SmdExportExt for Smd {
    fn export<W: Write>(&self, mut target: W) -> Result<(), io::Error> {
        // Write the header
        writeln!(target, "version 1")?;

        // Write the bone nodes
        writeln!(target, "nodes")?;
        for bone in &self.bones {
            writeln!(target, "{} \"{}\" -1", bone.id, bone.name)?;
        }
        writeln!(target, "end")?;

        // Add an idle animation frame for all the bones
        // TODO: Allow animation to be defined
        writeln!(target, "skeleton")?;
        for (num, frame) in &self.animation_frames {
            writeln!(target, "time {}", num)?;
            for (bone_id, bone) in &frame.bones {
                writeln!(target,
                    "{}   {} {} {}   {} {} {}",
                    bone_id,
                    bone.translation[0],
                    bone.translation[1],
                    bone.translation[2],
                    bone.rotation[0],
                    bone.rotation[1],
                    bone.rotation[2],
                )?;
            }
        }
        writeln!(target, "end")?;

        // Write the actual triangles
        writeln!(target, "triangles")?;
        for triangle in &self.triangles {
            // Write the material for this tri
            writeln!(target, "{}", triangle.material)?;

            // Write the triangle vertices
            for vertex in &triangle.vertices {
                // Parent bone
                write!(target, "{}   ", vertex.parent_bone)?;

                // Position, Normal, UV
                write!(target, "{} {} {}   ", vertex.position[0], vertex.position[1], vertex.position[2])?;
                write!(target, "{} {} {}   ", vertex.normal[0], vertex.normal[1], vertex.normal[2])?;
                write!(target, "{} {}   ", vertex.uv[0], vertex.uv[1])?;

                // Link amount
                write!(target, "{}  ", vertex.links.len())?;
                for link in &vertex.links {
                    // Individual link
                    write!(target, "{} {}  ", link.bone, link.weight)?;
                }

                // Finish the vertex off
                writeln!(target)?;
            }
        }
        writeln!(target, "end")?;

        Ok(())
    }
}
