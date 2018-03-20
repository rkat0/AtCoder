import Data.List (sort)

main :: IO()
main = do
    _ <- getLine
    as <- map read . words <$> getLine
    print . abs $ foldr1 (-) (sort as)
