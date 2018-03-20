import Control.Applicative
import Control.Monad
import Data.List
import Data.Function


main :: IO()
main = do
    n <- readLn
    csfs <- replicateM (n-1) (map read . words <$> getLine)
    ($ 0) . fix $ \loop i ->
        unless (i == n) $ do
            print $ foldl' time 0 (drop i csfs)
            loop (i+1)
    where
        time t [c,s,f] = c + s + f * k
            where k = max 0 ((t - s - 1) `div` f + 1)
