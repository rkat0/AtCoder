import Control.Monad

main :: IO()
main = do
    css <- replicateM 3 $ map read . words <$> getLine
    putStrLn $ calcAns css
        where
            calcAns [cs1,cs2,cs3] = if all (==0) (init ds1) && all (==0) (init ds2) then "Yes" else "No"
                where
                    cs1' = zipWith (-) cs1 cs3
                    cs2' = zipWith (-) cs2 cs3
                    ds1 = f cs1'
                    ds2 = f cs2'
                    f [a,b,c] = [a-c,b-c,c]
