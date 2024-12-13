use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DiskEntry {
    Free,
    File { id: u16 },
}

struct MemoryMap {
    entries: Vec<DiskEntry>,
}

impl MemoryMap {
    fn from_packed(packed: &str) -> Self {
        let mut id_number = 0;
        let mut entries = Vec::new();
        packed
            .chars()
            .enumerate()
            .into_iter()
            .for_each(|(i, num_blocks)| {
                let num_blocks = (num_blocks as u8 - b'0') as usize;
                let this_entries = if i % 2 == 0 {
                    let entries = vec![DiskEntry::File { id: id_number }; num_blocks];
                    id_number += 1;
                    entries
                } else {
                    vec![DiskEntry::Free; num_blocks]
                };
                entries.extend(this_entries);
            });

        Self { entries }
    }
}

impl std::fmt::Display for MemoryMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for entry in &self.entries {
            match entry {
                DiskEntry::Free => write!(f, ".")?,
                DiskEntry::File { id } => write!(f, "{}", id)?,
            };
        }

        writeln!(f)
    }
}

// Super not proud of my part1 solution lol
pub fn part1(input: &str) -> usize {
    let memory_map = MemoryMap::from_packed(input);
    let mut new_entries = Vec::new();

    let mut back_it = memory_map.entries.iter().enumerate().rev().fuse();
    let (mut back_i, mut n) = back_it.next().unwrap();

    for (i, entry) in memory_map.entries.iter().enumerate() {
        if i > back_i {
            break;
        }

        match entry {
            DiskEntry::Free => {
                // free space, find the next file entry
                let new_id = loop {
                    match n {
                        DiskEntry::Free => (),
                        DiskEntry::File { id } => break *id,
                    }
                    (back_i, n) = back_it.next().unwrap();
                };

                // now check if we aren't overlapping
                if i > back_i {
                    break;
                }
                new_entries.push(new_id);
                (back_i, n) = back_it.next().unwrap();
            }
            DiskEntry::File { id } => new_entries.push(*id),
        }
    }

    eprintln!("{}", new_entries.len());

    new_entries
        .into_iter()
        .enumerate()
        .map(|(i, n)| i * n as usize)
        .sum()
}

#[derive(Debug, Copy, Clone)]
struct BlocksAndIndex {
    num_blocks: u8,
    index: usize,
    id: u16,
}

fn display_blocks(blocks: &[BlocksAndIndex]) {
    let mut blocks: Vec<_> = blocks.iter().copied().collect();
    blocks.sort_by_key(|x| x.index);
    for (i, b) in blocks.iter().enumerate() {
        for _ in 0..b.num_blocks {
            print!("{}", b.id)
        }

        if let Some(next) = blocks.get(i + 1) {
            let diff = next.index - (b.index + b.num_blocks as usize);
            for _ in 0..diff {
                print!(".");
            }
        }
    }
    println!();
}

struct MemoryMapOptimized {
    files: Vec<BlocksAndIndex>,
    free: Vec<BlocksAndIndex>,
}

impl MemoryMapOptimized {
    fn from_packed(packed: &str) -> Self {
        let mut files = Vec::with_capacity(packed.len() / 2);
        let mut free = Vec::with_capacity(packed.len() / 2);
        let mut spaced_used = 0;
        for (i, num_blocks) in packed.char_indices() {
            let num_blocks = num_blocks as u8 - b'0';

            if i % 2 == 0 {
                files.push(BlocksAndIndex {
                    num_blocks,
                    index: spaced_used,
                    id: (i / 2).try_into().unwrap(),
                });
            } else {
                free.push(BlocksAndIndex {
                    num_blocks,
                    index: spaced_used,
                    id: 0,
                });
            }
            spaced_used += num_blocks as usize;
        }

        Self { files, free }
    }

    fn move_files(mut self) -> Vec<BlocksAndIndex> {
        // The free is sorted by index which is what we want. Find the leftmost block
        // that is large enough to fit.
        let mut free_blocks: Vec<_> = self.free;

        for last_block in self.files.iter_mut().rev() {
            // find the first leftmost free spot for with enough blocks
            let free_index = if let Some((free_index, free)) =
                free_blocks.iter_mut().find_position(|free| {
                    free.num_blocks >= last_block.num_blocks && free.index <= last_block.index
                }) {
                // found a free slot, move the file
                last_block.index = free.index;

                // now decrement the free slot and update it's new starting index
                // not the free array is still sorted because it will still never
                // move across the next free spot so we still want to pick it
                // if we find a file that can fit in the new number of blocks
                free.num_blocks -= last_block.num_blocks;
                free.index += last_block.num_blocks as usize;

                if free.num_blocks == 0 {
                    // remove it from the free list
                    Some(free_index)
                } else {
                    None
                }
            } else {
                None
            };

            if let Some(free_index) = free_index {
                free_blocks.remove(free_index);
            }
        }

        self.files
    }
}

pub fn part2(input: &str) -> usize {
    let memory_map = MemoryMapOptimized::from_packed(input);
    // display_blocks(&memory_map.files);
    let files_list = memory_map.move_files();
    // display_blocks(&files_list);
    files_list
        .into_iter()
        .map(|entry| {
            (entry.index..entry.index + entry.num_blocks as usize)
                .into_iter()
                .map(|index| index * entry.id as usize)
                .sum::<usize>()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"2333133121414131402"#;

    // #[test]
    // fn test_part1() {
    //     assert_eq!(part1(INPUT), 1928);
    // }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 2858);
    }
}
