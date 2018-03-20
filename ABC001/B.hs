main :: IO()
main = do
    m <- readLn
    putStrLn $ solve m
        where
            solve m
                | m < 100 = "00"
                | m < 1000 = '0':show h
                | m <= 5000 = show h
                | m >= 6000 && m <= 30000 = show (k + 50)
                | m >= 35000 && m <= 70000 = show ((k - 30) `div` 5 + 80)
                | m > 70000 = "89"
                | otherwise = "??" -- error
                where
                    h = m `div` 100
                    k = m `div` 1000
