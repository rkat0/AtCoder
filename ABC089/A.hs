main :: IO()
main = do
    n <- readLn
    print $ n `div` 3
