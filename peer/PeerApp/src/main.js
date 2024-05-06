//https://tauri.app/v1/guides/features/command/
//import { invoke } from '@tauri-apps/api/tauri'
const { invoke } = window.__TAURI__.tauri;

// #####################
// ##### FUNCTIONS #####
// function to add a message
function addMessage(message, divName, setTimeoutTime = 0) {
  //console.log('addMessage called to show: ', message);
  var messageElement = document.createElement('p');
  messageElement.textContent = message;
  divName.appendChild(messageElement);

  if (setTimeoutTime === 0) return;
  setTimeout(function () {
    divName.removeChild(messageElement);
  }, setTimeoutTime);
}


// ######################
// ##### BEGIN HERE #####

// ############################
// ##### INPUT FOR UPLOAD #####
// Create a file input element
let fileInput = document.createElement('input');
fileInput.type = 'file';
fileInput.multiple = true; // Allow multiple files to be selected


// #########################
// ##### UPLOAD BUTTON #####
document.getElementById('uploadButton').addEventListener('click', function () {
  console.log('Upload Files Button clicked');

  // Trigger the file input click event to open the file explorer
  fileInput.click();

  fileInput.addEventListener('change', function () {
    let files = fileInput.files; // This is a FileList object

    // Convert the FileList to an array of file names
    let filesNames = Array.from(files).map(file => file.name);

    console.log('filesNames:', filesNames);

    invoke('uploadFiles', { filesNames }).then((response) => { // Call the Rust function uploadFiles
      console.log(response);
    });
  });
});


// #########################
// ##### SEARCH BUTTON #####
document.getElementById('searchButton').addEventListener('click', function () {
  console.log('Search File to Download Button clicked');
  document.getElementById('searchForm').style.display = 'block';
});


// #######################
// ##### SEARCH FORM #####
document.getElementById('submitSearchForm').addEventListener('click', function (event) {
  // Prevent the form from submitting normally
  event.preventDefault();

  console.log('Search Form Submitted');

  // Get the input values
  var fileNameSubmitted = document.getElementById('fileNameSearchForm').value;
  var fileSizeSubmitted = document.getElementById('fileSizeSearchForm').value;

  console.log('File Name Submitted: ' + fileNameSubmitted);
  console.log('File Size Submitted: ' + fileSizeSubmitted);

  // Hide the form
  document.getElementById('searchForm').style.display = 'none';

  // Display a message
  let tmpStr = 'Search Form Submitted, File Name: ' + fileNameSubmitted + ', File Size: ' + fileSizeSubmitted;
  addMessage(tmpStr, document.getElementById('actions'), 5000);

  // example results from the rust function
  var results = [
    { name: 'file1.txt', size: 100 },
    { name: 'file2.txt', size: 200 },
    { name: 'file3.txt', size: 300 }
  ];


  // Display the search results
  var resultsList = document.createElement('ul');
  addMessage('Files Found, Click on File to Download:', document.getElementById('actions'));
  results.forEach(function (file) {
    var listItem = document.createElement('li');
    listItem.textContent = file.name + ' (' + file.size + ' bytes)';
    listItem.addEventListener('click', function () {
      console.log('File clicked:', file.name);
      // Display a message
      addMessage('Started downloading file: ' + file.name, document.getElementById('actions'), 5000);
    });
    resultsList.appendChild(listItem);
  });
  document.getElementById('actions').appendChild(resultsList);
});

