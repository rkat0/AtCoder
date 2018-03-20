import Control.Monad
import qualified Data.Map as M

main :: IO()
main = do
    [h,w] <- map read . words <$> getLine
    grid <- replicateM h getLine
    print $ calcAns grid h w
        where
            calcAns grid h w = if step > 0 then whites - step else -1
                where
                    step = dijkstra (toMap grid 1) (h,w)
                    toMap [] _ = M.empty
                    toMap (l:ls) n = toMap ls (n+1) `M.union` M.fromList (map (\(m,s) -> ((n,m),s)) $ zip [1..] l)
                    whites = sum $ map (length . filter (=='.')) grid

dijkstra g gp@(h,w) = dijk [(1,1)] (M.singleton (1,1) 1)
    where
        dijk [] _ = -1
        dijk (p:ps) d = if gp `elem` cont then d M.! p + 1 else dijk ps' d'
            where
                cont = filter (\(i,j) -> i>0 && i<=h && j>0 && j<=w && g M.! (i,j) == '.') (contiguous p)
                contiguous (i,j) = [(i-1,j),(i,j-1),(i+1,j),(i,j+1)]
                (ps',d') = foldr update (ps,d) cont
                    where
                        update p' (ps,d) = if M.member p' d then (ps,d) else (ps++[p'],M.insert p' (d M.! p + 1) d)
