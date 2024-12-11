import Data.Char (digitToInt)
import Data.Int (Int64)

blockHash :: Int64 -> Int64 -> Int64 -> Int64
blockHash 0 size id = id * quot ((size - 1) * size) 2
blockHash start size id = id * quot ((start + size - 1) * (start + size) - (start - 1) * start) 2

processBlock :: [Int64] -> Int64 -> Int64 -> Int64 -> Int64 -> Int64
processBlock [] current_pos left_id right_id hash = hash
processBlock [block] current_pos left_id right_id hash = hash + blockHash current_pos block left_id
processBlock (block:xs) current_pos left_id right_id hash = processGap xs (current_pos + block) (left_id + 1) right_id (hash + blockHash current_pos block left_id)

processGap :: [Int64] -> Int64 -> Int64 -> Int64 -> Int64 -> Int64
processGap [] current_pos left_id right_id hash = hash
processGap [gap, block] current_pos left_id right_id hash = hash + blockHash current_pos gap right_id
processGap disk current_pos left_id right_id hash
    | block > gap  = processBlock (init notfirst ++ [block - gap])     (current_pos + gap)   left_id right_id       (hash + blockHash current_pos gap right_id)
    | block == gap = processBlock ((init.init) notfirst)               (current_pos + gap)   left_id (right_id - 1) (hash + blockHash current_pos gap right_id)
    | block < gap  = processGap   ((gap - block):(init.init) notfirst) (current_pos + block) left_id (right_id - 1) (hash + blockHash current_pos block right_id)
    where
        (gap:notfirst) = disk
        block = last disk
    
part1 :: [Int64] -> Int64
part1 disk = processBlock disk 0 0 (((fromIntegral.length) disk - 1) `quot` 2) 0

extractGaps :: [Int64] -> [(Int64, Int64)]

main::IO()
main = do
  disk_map_str <- readFile "../diskmap.txt"
  -- let disk_map_str = "2333133121414131402"
  let disk_map :: [Int64] = map (fromIntegral.digitToInt) disk_map_str
  print (part1 disk_map)
