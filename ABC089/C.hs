import Control.Monad

main :: IO()
main = do
    n <- readLn
    ss <- replicateM n getLine
    print $ solve ss
        where
            solve ss = m*a*(r+c+h) + (m+a)*(r*(c+h) + c*h) + r*c*h
                where
                    m = count 'M' hs
                    a = count 'A' hs
                    r = count 'R' hs
                    c = count 'C' hs
                    h = count 'H' hs
                    hs = map head ss
                    count c s = length $ filter (== c) s
