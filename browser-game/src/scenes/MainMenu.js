import { Scene } from 'phaser';

export class MainMenu extends Scene
{
    constructor ()
    {
        super('MainMenu');
    }

    create ()
    {
        //this.add.image(512, 384, 'background');
        
        this.cameras.main.setBackgroundColor(0x648e9a);


        this.add.image(512, 380, 'logo').setScale(0.74);

        this.add.text(512, 685, 'Click to start', {
            fontFamily: 'Arial Black', fontSize: 38, color: '#ffffff',
            stroke: '#000000', strokeThickness: 8,
            align: 'center'
        }).setOrigin(0.5);

        this.input.on('pointerdown', () => {

            const leveldata = window.prompt("Enter the level json data", 
                window.globalLevelData ? JSON.stringify(window.globalLevelData) :
                '{"row":6,"cols":7,"map":"aaaa002844a222bc0aaaa0","player_row":2,"player_col":1}'
            );
            try {
                window.globalLevelData = JSON.parse(leveldata);
                this.scene.start('Game');
            } catch (e) {
                alert("Invalid JSON data");
            }

        });
    }
}
