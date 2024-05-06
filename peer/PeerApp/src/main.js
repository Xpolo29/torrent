//https://tauri.app/v1/guides/features/command/
//import { invoke } from '@tauri-apps/api/tauri'
const { invoke } = window.__TAURI__.tauri;

// #####################
// ##### FUNCTIONS #####
// function to add a message
function addMessage(message, divName, setTimeoutTime = 0, id = message) {
  //console.log('addMessage called to show: ', message);
  var messageElement = document.createElement('p');
  messageElement.textContent = message;
  messageElement.id = id;
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

  // Remove existing search results
  var existingResultsList = document.getElementById('resultsList');
  if (existingResultsList) {
    document.getElementById('actions').removeChild(existingResultsList);
    document.getElementById('actions').removeChild(document.getElementById('resultsListMessage'));
  }

  // Example results - real results should be taken from the backend
  var results = [
    { name: 'file1.txt', size: 100 },
    { name: 'file2.txt', size: 200 },
    { name: 'file3.txt', size: 300 }
  ];

  // Display the search results
  var resultsList = document.createElement('ul');
  resultsList.id = 'resultsList';  // Add an id to the results list
  addMessage('Files found, click on file to download:', document.getElementById('actions'), 0, 'resultsListMessage');
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


// #####################
// ##### DASHBOARD #####
var refreshRate = 1;  // Refresh rate in seconds
function populateDashboard() {
  // Example data - real data should be taken from the backend
  var data = [
    { name: 'file1.txt', downloadPercentage: 'Downloading... 50%', numberOfPeers: '13', leechingStatus: 'Leeching...' },
    { name: 'file2.txt', downloadPercentage: 'Downloading... 75%', numberOfPeers: '5', leechingStatus: 'Leeching...' },
    { name: 'file3.txt', downloadPercentage: 'Downloading... 15%', numberOfPeers: '8', leechingStatus: 'Not Leeching' }
  ];

  // Create a table
  var table = document.createElement('table');

  // Add table header
  var thead = document.createElement('thead');
  var headerRow = document.createElement('tr');
  ['File Name', 'Download Percentage', 'Number of Peers', 'Leeching Status'].forEach(function (header) {
    var th = document.createElement('th');
    th.textContent = header;
    headerRow.appendChild(th);
  });
  thead.appendChild(headerRow);
  table.appendChild(thead);

  // Add table body
  var tbody = document.createElement('tbody');
  data.forEach(function (file) {
    var row = document.createElement('tr');
    [file.name, file.downloadPercentage, file.numberOfPeers, file.leechingStatus].forEach(function (cell) {
      var td = document.createElement('td');
      td.textContent = cell;
      row.appendChild(td);
    });
    tbody.appendChild(row);
  });
  table.appendChild(tbody);

  // Add the table to the div
  var dashboardDiv = document.getElementById('torrentDashboard');
  dashboardDiv.innerHTML = '';  // Clear the div
  dashboardDiv.appendChild(table);

  // Add a timestamp
  addMessage('Updating every ' + refreshRate + ' seconds. Last updated: ' + new Date().toLocaleString(), dashboardDiv);}
// Update the dashboard every second
setInterval(populateDashboard, refreshRate*1000);
