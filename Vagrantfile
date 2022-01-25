Vagrant.configure("2") do |config|
    config.vm.define "default" do |default|
        default.vm.box = "debian/bullseye64"

        default.vm.provider :virtualbox do |virtualbox|
            virtualbox.memory = 2048
            virtualbox.cpus = 4
        end

        # Avoid installation of NFS on the guest (avoids elevated permissions requirement)
        default.vm.synced_folder './', '/vagrant', type: 'rsync', rsync__exclude: ['target*', '.vagrant*', '*.box']

        default.vm.provision "shell", path: "ci/provision.sh"

        if ENV["CUSTOM_ENV_GITLAB_CI"] == "true"
            # `gitlab-runner` is required for CI to be able to download previous stage artifacts: download_artifacts
            # https://docs.gitlab.com/runner/executors/custom.html
            # Copy `gitlab-runner` from runner host to guets VM.
            default.vm.provision "file", source: `which gitlab-runner`.strip(), destination: "/home/vagrant/gitlab-runner"
            default.vm.provision "shell", inline: "cp /home/vagrant/gitlab-runner /usr/local/bin/gitlab-runner", privileged: true
          end
    end
end
