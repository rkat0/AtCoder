import Control.Applicative
import Control.Monad
import Data.Function

main :: IO()
main = do
    [a,b] <- words <$> getLine
    let c = read (a ++ b) :: Int
    ($ 4) . fix $ \loop i ->
        case compare (i*i) c of
            LT -> loop (i+1)
            EQ -> putStrLn "Yes"
            GT -> putStrLn "No"
