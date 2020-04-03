#!/bin/bash
DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null && pwd )"

echo "Installing Pegasus..."

function unload() {
 # Remove the old daemon, if exists
  sudo launchctl unload /Library/LaunchAgents/com.black-dragon74.Pegasus.plist 2>/dev/null
  sudo pkill -f pegasus 2>/dev/null
  sudo rm /usr/local/bin/pegasus 2>/dev/null
  sudo rm /Library/LaunchAgents/com.black-dragon74.Pegasus.plist 2>/dev/null
}

# If only wanting to remove the daemon
if [ "$1" == "rem" ]; then
  echo "Removing the daemon.."
  unload
  exit 0
fi

# Copy to the destination
sudo cp "$DIR/pegasus" /usr/local/bin/
sudo chmod 755 /usr/local/bin/pegasus
sudo chown root:wheel /usr/local/bin/pegasus

# Copy the launchagent daemon
sudo cp "$DIR/com.black-dragon74.Pegasus.plist" /Library/LaunchAgents
sudo chmod 644 /Library/LaunchAgents/com.black-dragon74.Pegasus.plist
sudo chown root:wheel /Library/LaunchAgents/com.black-dragon74.Pegasus.plist

# Finally, load the daemon
sudo launchctl load /Library/LaunchAgents/com.black-dragon74.Pegasus.plist
