main :: IO()
main = do
    _ <- getLine
    ss <- getLine
    putStrLn $ if elem 'Y' ss then "Four" else "Three"
