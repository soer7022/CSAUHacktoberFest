main :: IO ()
main = do 
  let fib = 1 : 1 : zipWith (+) fib (tail fib)
  print $ fib !! 99