import Control.Monad
import qualified Data.IntSet as IS

main :: IO()
main = do
	n <- readLn
	as <- replicateM n readLn
	print $ solve as IS.empty
        where
            solve as s = IS.size $ foldr f s as
            f a s = if IS.member a s then IS.delete a s else IS.insert a s 
