main :: IO()
main = do
    a <- readLn
    b <- readLn
    c <- readLn
    x <- readLn
    print $ length [[d,e,f] | d <- [0..a], e <- [0..b], f <- [0..c], 500*d+100*e+50*f==x]
