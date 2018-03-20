import Control.Monad
import Control.Applicative

main :: IO()
main = do
	n <- readLn
	lrs <- replicateM n $ map read . words <$> getLine
	print $ sum $ map (\[l,r] -> r-l+1) lrs
