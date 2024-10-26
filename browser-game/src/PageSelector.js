// PAGE SELECTOR
 
import { LevelSelect } from "./scenes/LevelSelect";
 
// this class extends Sprite class
export class PageSelector extends Phaser.GameObjects.Sprite {
 
    //pageIndex;
    //parentScene;
 
    constructor(scene , x , y , key , pageIndex ) {
 
        super(scene, x, y, key);
       
        this.pageIndex = pageIndex;
        scene.add.existing(this);
        this.setInteractive();
        this.parentScene = scene;
 
        this.on('pointerdown', this.handlePointer);
          
       
    }
 
    handlePointer()  {
        this.parentScene.goToPage(this.pageIndex);
    }
 
     
}