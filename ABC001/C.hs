import Control.Applicative

main :: IO()
main = do
    [deg,dis] <- map read . words <$> getLine
    let dir = angle deg
    let pow = power dis
    putStrLn $ (if pow == 0 then "C" else dir) ++ ' ':show pow
        where
            angle x = case (x + 112) `mod` 3600 `div` 225 of
                0 -> "N"
                1 -> "NNE"
                2 -> "NE"
                3 -> "ENE"
                4 -> "E"
                5 -> "ESE"
                6 -> "SE"
                7 -> "SSE"
                8 -> "S"
                9 -> "SSW"
                10 -> "SW"
                11 -> "WSW"
                12 -> "W"
                13 -> "WNW"
                14 -> "NW"
                15 -> "NNW"
            power x
                | y <= 2 = 0
                | y <= 15 = 1
                | y <= 33 = 2
                | y <= 54 = 3
                | y <= 79 = 4
                | y <= 107 = 5
                | y <= 138 = 6
                | y <= 171 = 7
                | y <= 207 = 8
                | y <= 244 = 9
                | y <= 284 = 10
                | y <= 326 = 11
                | otherwise = 12
                where y = floor (fromIntegral x / 6 + 0.5)
