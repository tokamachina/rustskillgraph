extern crate byte_reader;
use byte_reader::Reader;
use std::io::Write;
use std::fs::File;

fn main() {
    let mut output = File::create("parsed_psg.txt").unwrap();
    let bytes = include_bytes!("passiveskillgraph.psg");
    let mut br = Reader::new(bytes);
    br.advance_by(54);
    let group_count = get_next_as_u32(&mut br);
    writeln!(&mut output, "groups: {}", group_count).unwrap();

    let mut groups: Vec<PsgGroup> = Vec::new();

    for _i in 0..group_count {
        let xloc = get_next_as_f32(&mut br);
        writeln!(&mut output, "xloc: {}", &xloc).unwrap();

        let yloc: f32 = get_next_as_f32(&mut br);
        writeln!(&mut output, "yloc: {}", &yloc).unwrap();

        br.advance_by(9);

        let node_count = get_next_as_u32(&mut br);
        writeln!(&mut output, "nodes: {}", &node_count).unwrap();

        let mut nodes: Vec<PsgNode> = Vec::new();

        for _j in 0..node_count {
            let node_id = get_next_as_u32(&mut br);
            writeln!(&mut output, "node id: {}", &node_id).unwrap();

            let orbit = get_next_as_u32(&mut br);
            writeln!(&mut output, "orbit: {}", &orbit).unwrap();

            let orbit_index = get_next_as_u32(&mut br);
            writeln!(&mut output, "orbit index: {}", &orbit_index).unwrap();

            let edge_count = get_next_as_u32(&mut br);
            writeln!(&mut output, "edge count: {}", &edge_count).unwrap();

            let mut edges: Vec<u32> = Vec::new();

            for _k in 0..edge_count {
                let new_edge: u32 = get_next_as_u32(&mut br);
                writeln!(&mut output, "new edge: {}", &new_edge).unwrap();
                edges.push(new_edge);
            }

            let new_node: PsgNode = PsgNode {
                node_id,
                orbit,
                orbit_index,
                edges
            };

            nodes.push(new_node);
            writeln!(&mut output, "node pushed").unwrap();
            writeln!(&mut output, "").unwrap();
        }

        let new_group: PsgGroup = PsgGroup {
            xloc,
            yloc,
            nodes
        };

        groups.push(new_group);
        writeln!(&mut output, "group {} pushed", _i+1).unwrap();
        writeln!(&mut output, "").unwrap();
        writeln!(&mut output, "").unwrap();
    }
    writeln!(&mut output, "{}", groups.len()).unwrap();
}

struct PsgNode {
    node_id: u32,
    orbit: u32,
    orbit_index: u32,
    edges: Vec<u32>
}

struct PsgGroup {
    xloc: f32,
    yloc: f32,
    nodes: Vec<PsgNode>
}

fn get_next_as_u32(br: &mut Reader) -> u32 {
    u32::from_le_bytes(
        [br.next().unwrap(),
        br.next().unwrap(),
        br.next().unwrap(),
        br.next().unwrap()]
    )
}

fn get_next_as_f32(br: &mut Reader) -> f32 {
    f32::from_le_bytes(
        [br.next().unwrap(),
        br.next().unwrap(),
        br.next().unwrap(),
        br.next().unwrap()]
    )
}