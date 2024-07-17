extern crate byte_reader;
use byte_reader::Reader;

fn main() {
    let bytes = include_bytes!("passiveskillgraph.psg");
    let mut br = Reader::new(bytes);
    br.advance_by(54);
    let group_count = get_next_as_u32(&mut br);
    println!("groups: {}", group_count);

    let mut groups: Vec<PsgGroup> = Vec::new();

    for _i in 0..group_count {
        let xloc = get_next_as_f32(&mut br);
        println!("xloc: {}", &xloc);

        let yloc: f32 = get_next_as_f32(&mut br);
        println!("yloc: {}", &yloc);

        br.advance_by(9);

        let node_count = get_next_as_u32(&mut br);
        println!("nodes: {}", &node_count);

        let mut nodes: Vec<PsgNode> = Vec::new();

        for _j in 0..node_count {
            let node_id = get_next_as_u32(&mut br);
            println!("node id: {}", &node_id);

            let orbit = get_next_as_u32(&mut br);
            println!("orbit: {}", &orbit);

            let orbit_index = get_next_as_u32(&mut br);
            println!("orbit index: {}", &orbit_index);

            let edge_count = get_next_as_u32(&mut br);
            println!("edge count: {}", &edge_count);

            let mut edges: Vec<u32> = Vec::new();

            for _k in 0..edge_count {
                let new_edge: u32 = get_next_as_u32(&mut br);
                println!("new edge: {}", &new_edge);
                edges.push(new_edge);
            }

            let new_node: PsgNode = PsgNode {
                node_id,
                orbit,
                orbit_index,
                edges
            };

            nodes.push(new_node);
            println!("node pushed");
            println!("");
        }

        let new_group: PsgGroup = PsgGroup {
            xloc,
            yloc,
            nodes
        };

        groups.push(new_group);
        println!("group {} pushed", _i+1);
        println!("");
        println!("");
    }
    println!("{}", groups.len());
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