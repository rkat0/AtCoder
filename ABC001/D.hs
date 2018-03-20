import Control.Applicative
import Control.Monad
import Data.List

main :: IO()
main = do
    n <- readLn
    ses <- replicateM n $ map read . (\s -> [take 4 s,drop 5 s]) <$> getLine
    putStr $ solve ses
        where
            solve = myShow . merge [] . sortOn head . map rnd
            rnd [s,e] = [s - s `mod` 5,if e' `mod` 100 == 60 then (e' `div` 100 + 1) * 100 else e']
                where e' = (e+4) `div` 5 * 5
            merge [] ([s,e]:ses) = merge [s,e] ses
            merge l [] = [l]
            merge [s0,e0] ([s,e]:ses)
                | e0 < s = [s0,e0] : merge [s,e] ses
                | e0 < e = merge [s0,e] ses
                | otherwise = merge [s0,e0] ses
            myShow [] = ""
            myShow ([s,e]:ses) = myShow' s ++ '-':myShow' e ++ '\n':myShow ses
                where
                    myShow' n = replicate (4 - length sn) '0' ++ sn
                        where sn = show n
