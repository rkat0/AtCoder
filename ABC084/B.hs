import Control.Applicative
import Data.Char

main :: IO()
main = do
    [a,b] <- map read . words <$> getLine
    s <- getLine
    if all isDigit (take a s) && s !! a == '-' && all isDigit (drop (a+1) s)
    then putStrLn "Yes"
    else putStrLn "No"
