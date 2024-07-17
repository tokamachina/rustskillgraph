extern crate byte_reader;
use byte_reader::Reader;

fn main() {
    let bytes = include_bytes!("passiveskillgraph.psg");
    let mut br = Reader::new(bytes);
    br.advance_by(54);
    let group_count = u32::from_le_bytes(
        [br.next().unwrap(),
        br.next().unwrap(),
        br.next().unwrap(),
        br.next().unwrap()]
    );
    println!("groups: {}", group_count);

    let mut groups: Vec<PsgGroup> = Vec::new();

    for _i in 0..group_count {
        let xloc = f32::from_le_bytes(
            [br.next().unwrap(),
            br.next().unwrap(),
            br.next().unwrap(),
            br.next().unwrap()]
        );
        println!("xloc: {}", &xloc);

        let yloc: f32 = f32::from_le_bytes(
            [br.next().unwrap(),
            br.next().unwrap(),
            br.next().unwrap(),
            br.next().unwrap()]
        );
        println!("yloc: {}", &yloc);

        br.advance_by(9);

        let node_count = u32::from_le_bytes(
            [br.next().unwrap(),
            br.next().unwrap(),
            br.next().unwrap(),
            br.next().unwrap()]
        );
        println!("nodes: {}", &node_count);

        let mut nodes: Vec<PsgNode> = Vec::new();

        for _j in 0..node_count {
            let node_id = u32::from_le_bytes(
                [br.next().unwrap(),
                br.next().unwrap(),
                br.next().unwrap(),
                br.next().unwrap()]
            );
            println!("node id: {}", &node_id);

            let orbit = u32::from_le_bytes(
                [br.next().unwrap(),
                br.next().unwrap(),
                br.next().unwrap(),
                br.next().unwrap()]
            );
            println!("orbit: {}", &orbit);

            let orbit_index = u32::from_le_bytes(
                [br.next().unwrap(),
                br.next().unwrap(),
                br.next().unwrap(),
                br.next().unwrap()]
            );
            println!("orbit index: {}", &orbit_index);

            let edge_count = u32::from_le_bytes(
                [br.next().unwrap(),
                br.next().unwrap(),
                br.next().unwrap(),
                br.next().unwrap()]
            );
            println!("edge count: {}", &edge_count);

            let mut edges: Vec<u32> = Vec::new();

            for _k in 0..edge_count {
                let new_edge: u32 = u32::from_le_bytes(
                    [br.next().unwrap(),
                    br.next().unwrap(),
                    br.next().unwrap(),
                    br.next().unwrap()]
                );
                println!("new edge: {}", &new_edge);
                edges.push(new_edge);
                if _k > 20 {break};
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


fn find_instances(byte_one: u8, byte_two: u8) -> Option<Vec<usize>> {
    let bytes = include_bytes!("passiveskillgraph.psg");
    let mut br = Reader::new(bytes);
    let mut inds: Vec<usize> = Vec::new();
    while let Some(byte) = br.next() {
        if byte == byte_one {
            if let Some(following_byte) = br.next() {
                if following_byte == byte_two {
                    inds.push(br.index-1);
                    println!("found byte sequence");
                }
            } else {
                break;
            }
        };
    }
    println!("{:?}", inds);
    match inds.is_empty() {
        false => Some(inds),
        _ => None,
    }
}

fn print_instances(byte_one: u8, byte_two: u8) {
    let bytes = include_bytes!("passiveskillgraph.psg");
    let mut br = Reader::new(bytes);
    br.read_until(vec![byte_one, byte_two]);
    for _i in 0..32 {
        let dat = [
            br.next().unwrap(), 
            br.next().unwrap(), 
            br.next().unwrap(), 
            br.next().unwrap()
            ];
        println!("{:02X?}", dat);
    }
    println!("");
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