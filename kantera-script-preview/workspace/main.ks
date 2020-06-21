(set framerate 10)

(set video
    (composite
        (vec (plain (path (rgb 0.0 0.0 1.0) (vec 10.0 (rgb 1.0 0.0 0.0) 'linear))) 'normal)
        (vec (plain (rgb 1.0 1.0 1.0)) 'normal (timed/add (sin 0.0 0.3 0.1) (sin 0.0 2.0 0.5)))))
