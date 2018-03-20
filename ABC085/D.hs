import Control.Applicative
import Control.Monad
import Data.List

main :: IO()
main = do
    [n,h] <- map read . words <$> getLine
    abt <- replicateM n $ map read . words <$> getLine
    print $ calcAns h (transpose abt)
        where
            calcAns h [as,bs]
                | sum bs' >= h = partial h bs'
                | otherwise = length bs' + ceiling (fromIntegral (h - sum bs') / fromIntegral amax)
                where
                    amax = maximum as
                    bs' = sortBy (flip compare) $ filter (> amax) bs
            partial h l = partial' h l 0
            partial' h (b:bs) i
                | h <= b = i + 1
                | otherwise = partial' (h-b) bs (i+1)
