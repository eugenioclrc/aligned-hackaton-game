import { Scene } from 'phaser';

export class MainMenu extends Scene
{
    constructor ()
    {
        super('MainMenu');
    }

    create ()
    {
        this.add.image(512, 384, 'background');

        this.add.image(512, 300, 'logo');

        this.add.text(512, 460, 'Click to start', {
            fontFamily: 'Arial Black', fontSize: 38, color: '#ffffff',
            stroke: '#000000', strokeThickness: 8,
            align: 'center'
        }).setOrigin(0.5);

        this.input.on('pointerdown', () => {

            const leveldata = window.prompt("Enter the level json data", 
                window.globalLevelData ? JSON.stringify(window.globalLevelData) :
                '{"row":6,"cols":7,"map":"aaaa002844a222bc0aaaa0","playerRow":2,"playerCol":1}'
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
