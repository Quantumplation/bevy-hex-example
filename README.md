# Bevy 3D Hex Example

I put together a small 3D hex grid for a game I was working on, and there was some interest in how I achieved that.  I extracted that code into this repository for people to learn from.

It was heavily inspired by two main resources:
 - [Redblobgames' excellent writeup on hex grids](https://www.redblobgames.com/grids/hexagons/#map-storage)
 - [Catlike Coding's tutorial series on hex grids in unity](https://catlikecoding.com/unity/tutorials/hex-map/)

## Outline

I recommend perusing the source in this order:

 - hex.rs: Simple hex-coordinate operations, such as neighbors and the like
 - geometry.rs: Generates 3D points for a simple hex mesh
 - main.rs: Wires this together to set up a bevy scene 

## Future

I don't have a lot of free time, so I didn't wrap this up in a bevy plugin for use by the community.  If there's substantial interest, however, I might be talked into that.
