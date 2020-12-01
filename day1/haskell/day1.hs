
load :: String -> [Int]
load = map read . lines

listWhichAddTo2020 :: [[Int]] -> [Int]
listWhichAddTo2020 = head . filter addsTo2020

addsTo2020 :: [Int] -> Bool
addsTo2020 xs = sum xs == 2020

part1 :: [Int] -> Int
part1 input = product $ listWhichAddTo2020 sums
  where
    sums = do
      x <- input
      y <- input
      return [x, y]

part2 :: [Int] -> Int
part2 input = product $ listWhichAddTo2020 sums
  where
    sums = do
      x <- input
      y <- input
      z <- input
      return [x, y, z]


main = do
  input <- fmap load (readFile "../input.txt")
  putStrLn $ show $ part1 input
  putStrLn $ show $ part2 input

