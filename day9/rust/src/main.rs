use itertools::Itertools;

fn block_hash(start_pos: u32, size: u32, id: u32) -> u64 {
    let start_pos = start_pos as u64;
    let size = size as u64;
    let id = id as u64;

    if start_pos > 0 {
        id * ((start_pos + size - 1) * (start_pos + size) - (start_pos - 1) * start_pos) / 2
    } else {
        id * ((start_pos + size - 1) * (start_pos + size)) / 2
    }
}

fn part1(disk_map: &Vec<u32>) -> u64 {
    let mut disk_map = disk_map.clone();
    let mut current_pos = 0;
    let mut current_idx = 0;
    let mut hash = 0;
    loop {
        if current_idx >= disk_map.len() {
            break;
        }
        let size = disk_map[current_idx];

        if current_idx % 2 == 0 {
            let id = current_idx as u32 / 2;
            hash += block_hash(current_pos, size, id);
            current_pos += size;
        } else {
            let mut gap = size;
            while gap > 0 {
                let last = disk_map.last().unwrap();
                let last_id = (disk_map.len() - 1) as u32 / 2;
                if gap >= *last {
                    gap -= last;
                    hash += block_hash(current_pos, *last, last_id);
                    current_pos += last;
                    disk_map.pop(); // pop item
                    disk_map.pop(); // pop gap
                } else {
                    hash += block_hash(current_pos, gap, last_id);
                    current_pos += gap;
                    *disk_map.last_mut().unwrap() -= gap;
                    break;
                }
            }
        }
        current_idx += 1;
    }
    hash
}

fn part2(disk_map: &Vec<u32>) -> u64 {
    let disk_map = disk_map.clone();
    let mut hash = 0;
    let mut indices: Vec<u32> = vec![0];
    let mut gaps: Vec<(u32, u32)> = Vec::new();

    for (id, (block, gap)) in disk_map.iter().tuples().enumerate() {
        indices.push(indices[id] + block + gap);
        let prev_gap = gaps.last().unwrap_or(&(0, 0));
        gaps.push((prev_gap.0 + prev_gap.1 + block, *gap));
    }

    for cand_id in (0..((disk_map.len() + 1) / 2)).rev() {
        let cand_map_idx = cand_id * 2;
        let cand_size = disk_map[cand_map_idx];
        let mut new_cand_idx = indices[cand_id];
        for (gap_idx, gap) in gaps.iter_mut().enumerate() {
            if indices[cand_id] < gap.0 {
                // no gaps to the left
                break;
            }
            if cand_size <= gap.1 {
                new_cand_idx = gap.0;
                gap.1 -= cand_size;
                gap.0 += cand_size;
                if gap.1 == 0 {
                    gaps.remove(gap_idx); // theoretically this should be O(1), but rust doesn't have a suitable linked list
                }
                break;
            }
        }
        hash += block_hash(new_cand_idx, cand_size, cand_id as u32);
    }
    hash
}

fn main() {
    let disk_map: Vec<u32> = std::fs::read_to_string("../diskmap.txt")
        .unwrap()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect();
    println!("Part 1: {}", part1(&disk_map));
    println!("Part 2: {}", part2(&disk_map));
}
