#[cfg(test)]
mod tests {
    use r64drive::*;
    #[test]
    fn test_send_cmd() {
        let cmd_id = Commands::VersionRequest;

        let r64d = R64Drive::new();
        r64d.send_cmd(cmd_id, &[]);
    }
}
