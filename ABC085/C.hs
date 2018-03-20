import Control.Applicative

main :: IO()
main = do
    [n,y] <- map read . words <$> getLine
    putStrLn $ myshow [[a,b,n-a-b] | a <- [0..n], b <- [0..n], a+b<=n && 9000*a+4000*b==y-1000*n]
        where
            myshow [] = "-1 -1 -1"
            myshow (x:_) = tail $ concatMap ((' ':) . show) x
