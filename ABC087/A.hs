main :: IO()
main = do
    x <- readLn
    a <- readLn
    b <- readLn
    print $ (x - a) `mod` b
