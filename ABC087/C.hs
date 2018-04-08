import Control.Applicative

main :: IO()
main = do
    _ <- getLine
    a1 <- map read . words <$> getLine
    a2 <- map read . words <$> getLine
    print $ maxsum (zipWith (-) (tail a1) (init a2)) + head a1 + sum a2
        where
            maxsum a = maxsum' a 0 0
            maxsum' [] _ m = m
            maxsum' (a:as) s m = maxsum' as s' $ if s' > m then s' else m
                where s' = a + s
