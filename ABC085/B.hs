import Control.Monad
import Data.Set

main :: IO()
main = do
    n <- readLn
    ds <- replicateM n (readLn :: IO Int)
    print $ length $ fromList ds
