//https://tauri.app/v1/guides/features/command/
//import { invoke } from '@tauri-apps/api/tauri'
const { invoke } = window.__TAURI__.tauri;


// connect button
document.getElementById('connectButton').addEventListener('click', function () {
  console.log('connectButton clicked');
  invoke('openConnection').then((response) => {
    console.log(response);
  });
});

// disconnect button
document.getElementById('disconnectButton').addEventListener('click', function () {
  console.log('disconnectButton clicked');
  invoke('closeConnection').then((response) => {
    console.log(response);
  });
});

// read input in filesNames input and call rust function uploadFiles
document.getElementById('uploadButton').addEventListener('click', function () {
  console.log('uploadButton clicked');
  let filesNames = document.getElementById('filesNames').value;
  console.log('filesNames:', filesNames);
  invoke('uploadFiles', { filesNames }).then((response) => {
    console.log(response);
  });
});

// read input in filesNames input and call rust function downloadFiles
document.getElementById('downloadButton').addEventListener('click', function () {
  console.log('downloadButton clicked');
  let filesNames = document.getElementById('filesNames').value;
  console.log('filesNames:', filesNames);
  invoke('downloadFiles', { filesNames }).then((response) => {
    console.log(response);
  });
});
