main :: IO()
main = do
    n <- readLn
    a <- readLn
    putStrLn $ if n `mod` 500 <= a then "Yes" else "No"
