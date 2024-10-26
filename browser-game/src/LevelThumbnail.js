// LEVEL THUMBNAIL
 
// this class extends Sprite class
export class LevelThumbnail extends Phaser.GameObjects.Sprite {
 
    //levelNumber : number;
   //locked : boolean;
 
    constructor(scene , x , y , key , level , locked ) {
        super(scene, x, y, key);
        scene.add.existing(this);
        this.levelNumber = level;
        this.locked = locked;
        this.setFrame(locked ? 0 : 1);
    }
}