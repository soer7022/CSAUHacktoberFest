data Number = S Number | Z 
  deriving Show

add :: Number -> Number -> Number
add n (S m') = add (S n) m'
add n Z = n

main :: IO ()
main = do 
  let four = S (S (S (S Z)))
  let three = S (S (S Z))
  print $ add four three