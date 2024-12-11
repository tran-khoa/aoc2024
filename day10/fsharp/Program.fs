open System
open System.Collections.Generic
open System.IO
open System.Numerics

type Position = Complex
type ComplexMap = IDictionary<Complex, int>
type PositionSet = HashSet<Position>
type UniqueEndsCache = Dictionary<Position, PositionSet>
type PathCache = Dictionary<Position, int>


let fileContent = File.ReadAllText("../map.txt")

let directions =
    [| Complex(0.0, 1.0)
       Complex(0.0, -1.0)
       Complex(1.0, 0.0)
       Complex(-1.0, 0.0) |]

let map: ComplexMap =
    fileContent.Split("\n")
    |> Array.map _.ToCharArray()
    |> Array.mapi (fun i ->
        Array.mapi (fun j value -> (Complex(double i, double j), int (Char.GetNumericValue(value)))))
    |> Array.collect id
    |> dict

let rec dfs_unique_ends (map: ComplexMap) (pos: Position) (cache: UniqueEndsCache) : UniqueEndsCache =
    if cache.ContainsKey(pos) then
        cache
    elif map.[pos] = 9 then
        cache.Add(pos, PositionSet([ pos ]))
        cache
    else
        let mutable cache = cache
        let ends = PositionSet()

        for direction in directions do
            let next_pos = pos + direction

            if map.ContainsKey(next_pos) && map.[next_pos] = (map.[pos] + 1) then
                cache <- dfs_unique_ends map next_pos cache
                ends.UnionWith(cache.[next_pos])

        cache.Add(pos, ends)
        cache

let part1 (map: ComplexMap) =
    map
    |> Seq.map (fun kvp ->
        if kvp.Value = 0 then
            (dfs_unique_ends map kvp.Key (UniqueEndsCache())).[kvp.Key].Count
        else
            0)
    |> Seq.sum


let rec dfs_paths (map: ComplexMap) (pos: Position) (cache: PathCache) : PathCache =
    if cache.ContainsKey(pos) then
        cache
    elif map.[pos] = 9 then
        cache.Add(pos, 1)
        cache
    else
        let mutable cache = cache
        let mutable paths = 0

        for direction in directions do
            let next_pos = pos + direction

            if map.ContainsKey(next_pos) && map.[next_pos] = (map.[pos] + 1) then
                cache <- dfs_paths map next_pos cache
                paths <- paths + cache.[next_pos]

        cache.Add(pos, paths)
        cache

let part2 (map: ComplexMap) =
    map
    |> Seq.map (fun kvp ->
        if kvp.Value = 0 then
            (dfs_paths map kvp.Key (PathCache())).[kvp.Key]
        else
            0)
    |> Seq.sum

printfn $"Part 1: %d{part1 map}"
printfn $"Part 2: %d{part2 map}"
