# symbolDescriptions
## composite
composite

```kanteraScript
(composite
  (vec render_1 'normal)
  (vec render_2 'normal))
```

## plain
plain

```kanteraScript
(plain (rgb 1.0 0.0 0.0))
```

## path
path

```kanteraScript
(path initial_value
  (vec dtime second_value interpolation)
  ...)
```

## audio_clip
audio_clip

```kanteraScript
(audio_clip
  audio_render
  1.0 ; gain
  0.0 ; pan (-1.0 ~ 1.0)
  0.0 ; start (sec)
  10.0 ; duration (sec)
  1.0 ; pitch
  0.0 ; fadein (sec)
  0.0 ; fadeout (sec)
  )
```
