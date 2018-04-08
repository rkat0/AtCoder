import Control.Applicative
import Control.Monad
import Data.Function

main :: IO()
main = do
    n <- readLn
    plan <- replicateM n $ map read . words <$> getLine
    putStrLn $ possible plan (0,0,0)
        where
            possible [] _ = "Yes"
            possible ([t',x',y']:plan) (t,x,y)
                | d >= 0 && even d = possible plan (t',x',y')
                | otherwise = "No"
                where d = t' - t - abs (x' - x) - abs (y' - y)
