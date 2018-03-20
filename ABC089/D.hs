import Control.Applicative
import Control.Monad
import qualified Data.IntMap as IM
import qualified Data.IntSet as IS

main :: IO()
main = do
    [h,w,d] <- map read . words <$> getLine
    ass <- replicateM h $ map read . words <$> getLine
    q <- readLn
    lrs <- replicateM q $ map read . words <$> getLine
    let m = fromll ass IM.empty 0
    mapM_ (print . solve m d (h*w)) lrs
    where
        fromll [] m _ = m
        fromll (xs:xss) m i = fromll xss (froml xs m i 0) (i+1)
        froml [] m _ _ = m
        froml (x:xs) m i j = froml xs (IM.insert x (i,j) m) i (j+1)
        solve m d u [l,r] = if r == l then 0 else table r - table l
            where
                table = (IM.fromSet f (IS.fromList [1..u]) IM.!)
                f z
                    | y == 0 = 0
                    | otherwise = cost (z-d) z + table (z-d)
                    where
                        y = snd $ conv z
                        conv x = ((x-1) `mod` d + 1,(x-1) `div` d)
                cost a b = abs (ax-bx) + abs (ay-by)
                    where
                        (ax,ay) = m IM.! a
                        (bx,by) = m IM.! b
