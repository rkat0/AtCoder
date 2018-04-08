import Control.Applicative

main :: IO()
main = do
    [a,b] <- map read . words <$> getLine
    putStrLn $ if odd (a * b) then "Odd" else "Even"
