main :: IO()
main = do
	s <- getLine
	putStrLn $ if '9' `elem` s then "Yes" else "No"
