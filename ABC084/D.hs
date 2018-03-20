import Data.IntMap.Lazy (IntMap,fromSet,(!))
import qualified Data.IntSet as IntSet
import Data.Ix
import Data.Function

import Debug.Trace

main :: IO()
main = do
    _ <- getLine
    lrs <- getContents
    mapM_ (print . countRange . map read . words) (lines lrs)
        where
            countRange [l,r]
                | l <= r = memocount r - memocount (l - 1)
                | otherwise = 0
            memocount :: Int -> Int
            memocount = ((fromSet count $ IntSet.fromList [0..100000]) !)
                where
                    count 0 = 0
                    count 1 = 0
                    count n = (if is2017Like n then 1 else 0) + memocount (n - 1)
            is2017Like 2 = False
            is2017Like 3 = True
            is2017Like n = mod n 4 == 1 && isPrime n && isPrime ((n + 1) `div` 2)
            isPrime n = null [i | i <- takeWhile (\x -> x * x <= n) primes, mod n i == 0]
            primes = 2 : 3 : primes'
                where
                    primes' = 5 : f 1 7 primes'
                    f m s (p : ps) = [n | n <- ns, gcd m n == 1] ++ f (m * p) (p * p) ps
                        where ns = [x + y | x <- [s, s + 6 .. p * p - 2], y <- [0, 4]]
