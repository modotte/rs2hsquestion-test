module Main where

import Control.Monad (void)
import Rs2hsquestion (askConfirmation)

main :: IO ()
main = do
  answer <- askConfirmation "Do you want to continue?"
  print answer
